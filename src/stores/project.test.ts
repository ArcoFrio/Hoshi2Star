import { beforeEach, describe, expect, it, vi } from "vitest";
import { mockIPC } from "@tauri-apps/api/mocks";

const toastSuccess = vi.fn();
const toastWarning = vi.fn();
vi.mock("sonner", () => ({
  toast: {
    success: (...args: unknown[]) => toastSuccess(...args),
    warning: (...args: unknown[]) => toastWarning(...args),
    error: vi.fn(),
  },
}));

import {
  openProject,
  loadAllProjects,
  useProjectStore,
} from "@/stores/project";
import type { CoverageView, Project, ProjectStats } from "@/lib/types";
import { useSettingsStore } from "@/stores/settings";
import { useCoverageStore } from "@/stores/coverage";

const initialState = useProjectStore.getState();

const PROJECT: Project = {
  id: "p1",
  name: "Test Game",
  engine: "mv_mz",
  gamePath: "/tmp/game",
  sourceLang: "ja",
  targetLang: "fr",
  createdAt: "2026-07-01",
  updatedAt: "2026-07-01",
} as Project;

const STATS: ProjectStats = {
  fileCount: 1,
  totalSegments: 10,
  untranslatedCount: 8,
  translatedCount: 2,
  needsReviewCount: 0,
  reviewedCount: 0,
};

/** Nothing optional to offer: the post-open coverage step is a no-op. */
const EMPTY_COVERAGE: CoverageView = {
  supported: true,
  hasDecisions: false,
  reconstructed: false,
  driftedFiles: [],
  pluginsUnreadable: null,
  categories: [],
  coreWarnings: [],
  notCovered: [],
};

const CATEGORY = {
  title: "Equipment slot names",
  group: "system" as const,
  enabled: false,
  decided: false,
  visibleIn: "Equip screen",
  consequences: [],
  constraints: [],
  translatedCount: 0,
  samples: [],
  references: [],
  referenceCount: 0,
  blockedReason: null,
};

const OFFERED: CoverageView = {
  ...EMPTY_COVERAGE,
  categories: [
    {
      ...CATEGORY,
      key: "system.equip_types",
      state: "verified",
      defaultEnabled: true,
      segmentCount: 2,
    },
    {
      ...CATEGORY,
      key: "system.skill_types",
      state: "blocked",
      defaultEnabled: false,
      segmentCount: 3,
      blockedReason: "looked up by name",
    },
  ],
};

beforeEach(() => {
  toastSuccess.mockClear();
  toastWarning.mockClear();
  useProjectStore.setState(initialState, true);
  useCoverageStore.setState({ open: false, view: null, error: null });
});

describe("project store — openProject thunk", () => {
  it("registers the project, sets it active and loads files + stats", async () => {
    let openArgs: unknown;
    mockIPC((cmd, args) => {
      switch (cmd) {
        case "open_project":
          openArgs = args;
          return { project: PROJECT, wasRestored: false };
        case "get_source_files":
          return [];
        case "get_project_stats":
          return STATS;
        case "scan_text_coverage":
          return EMPTY_COVERAGE;
        default:
          throw new Error(`unexpected command: ${cmd}`);
      }
    });

    const result = await openProject("/tmp/game");

    expect(result.project.id).toBe("p1");
    const s = useProjectStore.getState();
    expect(s.projects).toHaveLength(1);
    expect(s.activeProjectId).toBe("p1");
    expect(s.activeProjectStats).toEqual(STATS);
    expect(openArgs as Record<string, unknown>).toMatchObject({
      path: "/tmp/game",
      sourceLang: useSettingsStore.getState().settings.defaultSourceLang,
      targetLang: useSettingsStore.getState().settings.defaultTargetLang,
    });
    expect(toastSuccess).toHaveBeenCalled();
  });

  it("restores an existing project without duplicating it", async () => {
    mockIPC((cmd) => {
      switch (cmd) {
        case "open_project":
          return { project: PROJECT, wasRestored: true };
        case "get_source_files":
          return [];
        case "get_project_stats":
          return STATS;
        case "scan_text_coverage":
          return EMPTY_COVERAGE;
        default:
          throw new Error(`unexpected command: ${cmd}`);
      }
    });

    await openProject("/tmp/game");
    expect(useProjectStore.getState().projects).toHaveLength(1);
  });

  it("updates a restored project without duplicating its library card", async () => {
    useProjectStore.setState({ projects: [PROJECT] });
    mockIPC((cmd) => {
      if (cmd === "open_project") {
        return {
          project: { ...PROJECT, name: "Updated title" },
          wasRestored: true,
        };
      }
      if (cmd === "get_source_files") return [];
      if (cmd === "get_project_stats") return STATS;
      if (cmd === "scan_text_coverage") return EMPTY_COVERAGE;
      throw new Error(`unexpected command: ${cmd}`);
    });

    await openProject("/tmp/game");

    expect(useProjectStore.getState().projects).toHaveLength(1);
    expect(useProjectStore.getState().projects[0].name).toBe("Updated title");
  });

  it("enables only verified text categories after a fresh MV/MZ extraction", async () => {
    let applied: unknown;
    mockIPC((cmd, args) => {
      switch (cmd) {
        case "open_project":
          return { project: PROJECT, wasRestored: false };
        case "get_source_files":
          return [];
        case "get_project_stats":
          return STATS;
        case "scan_text_coverage":
          return OFFERED;
        case "set_text_categories":
          applied = args;
          return { ...OFFERED, hasDecisions: true };
        default:
          throw new Error(`unexpected command: ${cmd}`);
      }
    });

    await openProject("/tmp/game");

    expect(applied).toEqual({
      projectId: "p1",
      changes: [{ categoryKey: "system.equip_types", enabled: true }],
    });
    expect(useCoverageStore.getState().open).toBe(false);
  });

  it("opens the coverage review instead of enabling anything for a restored project", async () => {
    mockIPC((cmd) => {
      switch (cmd) {
        case "open_project":
          return { project: PROJECT, wasRestored: true };
        case "get_source_files":
          return [];
        case "get_project_stats":
          return STATS;
        case "scan_text_coverage":
          return OFFERED;
        default:
          throw new Error(`unexpected command: ${cmd}`);
      }
    });

    await openProject("/tmp/game");

    expect(useCoverageStore.getState().open).toBe(true);
    expect(useCoverageStore.getState().view?.categories).toHaveLength(2);
  });

  it("never fails opening when the coverage scan fails", async () => {
    mockIPC((cmd) => {
      switch (cmd) {
        case "open_project":
          return { project: PROJECT, wasRestored: false };
        case "get_source_files":
          return [];
        case "get_project_stats":
          return STATS;
        default:
          throw new Error("scan exploded");
      }
    });

    const result = await openProject("/tmp/game");

    expect(result.project.id).toBe("p1");
    expect(toastWarning).toHaveBeenCalled();
  });

  it("propagates a backend failure to the caller without touching the store", async () => {
    mockIPC(() => {
      throw new Error("could not identify game engine");
    });

    await expect(openProject("/tmp/nope")).rejects.toThrow();
    expect(useProjectStore.getState().projects).toHaveLength(0);
    expect(useProjectStore.getState().activeProjectId).toBeNull();
  });
});

describe("project store — loadAllProjects", () => {
  it("merges DB projects without duplicating existing ones", async () => {
    useProjectStore.getState().addProject(PROJECT);
    mockIPC((cmd) => {
      if (cmd === "list_projects") {
        return [PROJECT, { ...PROJECT, id: "p2", name: "Other" }];
      }
      throw new Error(`unexpected command: ${cmd}`);
    });

    await loadAllProjects();
    const ids = useProjectStore.getState().projects.map((p) => p.id);
    expect(ids).toEqual(["p1", "p2"]);
  });
});
