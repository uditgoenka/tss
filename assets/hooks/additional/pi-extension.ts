import type { ExtensionAPI } from "@earendil-works/pi-coding-agent";
import { isToolCallEventType } from "@earendil-works/pi-coding-agent";

function shellQuote(value: string): string {
  return `'${value.replace(/'/g, `'\"'\"'`)}'`;
}

export default function tssExtension(pi: ExtensionAPI) {
  pi.on("tool_call", async (event) => {
    if (!isToolCallEventType("bash", event)) return;

    const command = event.input.command;
    if (
      typeof command !== "string" ||
      !command.trim() ||
      command.trim().startsWith("tss ") ||
      command.trim().startsWith("env TSS_AGENT=")
    ) {
      return;
    }

    event.input.command = `env TSS_AGENT=pi-dev tss run -- bash -lc ${shellQuote(command)}`;
  });
}
