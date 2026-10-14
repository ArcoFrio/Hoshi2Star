import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";
import type { CategoryChange, CoverageView } from "@/lib/types";

// ---------------------------------------------------------------------------
// Optional text coverage (MV/MZ plugin & system UI text)
// ---------------------------------------------------------------------------

interface CoverageState {
  open: boolean;
  loading: boolean;
  view: CoverageView | null;
  error: string | null;
  setOpen: (open: boolean) => void;
  setView: (view: CoverageView | null) => void;
}

export const useCoverageStore = create<CoverageState>()((set) => ({
  open: false,
  loading: false,
  view: null,
  error: null,
  setOpen: (open) => set({ open }),
  setView: (view) => set({ view }),
}));

export const useCoverageOpen = () => useCoverageStore((s) => s.open);

/** Scan the project and open the coverage dialog. */
export async function openCoverageDialog(projectId: string): Promise<void> {
  useCoverageStore.setState({ open: true, loading: true, error: null });
  try {
    const view = await invoke<CoverageView>("scan_text_coverage", {
      projectId,
    });
    useCoverageStore.setState({ view, loading: false });
  } catch (err) {
    useCoverageStore.setState({ loading: false, error: String(err) });
  }
}

/** Apply decisions; the backend re-scans and validates every change. */
export async function applyCoverageChanges(
  projectId: string,
  changes: CategoryChange[],
): Promise<CoverageView> {
  useCoverageStore.setState({ loading: true, error: null });
  try {
    const view = await invoke<CoverageView>("set_text_categories", {
      projectId,
      changes,
    });
    useCoverageStore.setState({ view, loading: false });
    return view;
  } catch (err) {
    useCoverageStore.setState({ loading: false, error: String(err) });
    throw err;
  }
}

/** Outcome of the post-open step, used for the user-facing toast. */
export type PostOpenCoverage =
  | { kind: "none" }
  | { kind: "enabled"; segments: number }
  | { kind: "review" };

/**
 * After opening an MV/MZ project:
 * - fresh extraction → enable every *verified* category silently (they are
 *   display-only and referenced nowhere);
 * - restored project that never decided → open the dialog so the user reviews
 *   the newly available categories (nothing is enabled behind their back).
 */
export async function applyPostOpenCoverage(
  projectId: string,
  wasRestored: boolean,
): Promise<PostOpenCoverage> {
  const view = await invoke<CoverageView>("scan_text_coverage", { projectId });
  if (!view.supported || view.hasDecisions || view.categories.length === 0) {
    return { kind: "none" };
  }
  if (wasRestored) {
    useCoverageStore.setState({ view, open: true, error: null });
    return { kind: "review" };
  }
  const defaults = view.categories.filter(
    (category) => category.state === "verified" && category.defaultEnabled,
  );
  if (defaults.length === 0) return { kind: "none" };
  const applied = await invoke<CoverageView>("set_text_categories", {
    projectId,
    changes: defaults.map((category) => ({
      categoryKey: category.key,
      enabled: true,
    })),
  });
  useCoverageStore.setState({ view: applied });
  return {
    kind: "enabled",
    segments: defaults.reduce((sum, category) => sum + category.segmentCount, 0),
  };
}
