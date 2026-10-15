import { cn } from "@/lib/utils";

export function Hoshi2StarMark({
  compact = false,
  className,
}: {
  compact?: boolean;
  className?: string;
}) {
  return (
    <div
      className={cn("flex min-w-0 items-center gap-2.5 select-none", className)}
      aria-label="Hoshi2Star — 星 → ★"
    >
      <span
        className="brand-seal relative grid size-9 shrink-0 place-items-center rounded-full"
        aria-hidden="true"
      >
        <span className="text-[15px] leading-none">星</span>
      </span>
      {!compact && (
        <span className="min-w-0 flex-1 leading-none">
          <span className="block truncate text-sm font-semibold tracking-[-0.02em]">
            Hoshi2Star
          </span>
          <span className="mt-1 block truncate text-[9px] font-medium tracking-[0.18em] text-muted-foreground">
            星 → ★
          </span>
        </span>
      )}
    </div>
  );
}
