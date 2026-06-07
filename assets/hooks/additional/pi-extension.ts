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
      alreadyOwned(command) ||
      command.trim().startsWith("env TSS_AGENT=") ||
      command.trim().startsWith("TSS_AGENT=") ||
      command.trim().startsWith("TSS_BYPASS=1") ||
      command.trim().startsWith("env TSS_BYPASS=1")
    ) {
      return;
    }

    event.input.command = `env TSS_AGENT=pi-dev tss run -- bash -lc ${shellQuote(command)}`;
  });
}

function alreadyOwned(command: string): boolean {
  const parts = shellWords(command);
  let index = parts[0] === "env" ? 1 : 0;
  if (parts[0] === "env") {
    while (index < parts.length) {
      const part = parts[index];
      if (part === "--") {
        index += 1;
        break;
      }
      if (part === "-S" || part === "--split-string") {
        return alreadyOwned(parts[index + 1] || "");
      }
      if (["-u", "-C", "-S", "--unset", "--chdir", "--split-string"].includes(part)) {
        index += 2;
        continue;
      }
      if (part.startsWith("-") || part.includes("=")) {
        index += 1;
        continue;
      }
      break;
    }
  }
  while (index < parts.length && parts[index].includes("=") && !parts[index].startsWith("-")) index += 1;
  if (parts[index] === "command") index += 1;
  const executable = String(parts[index] || "").replace(/\\/g, "/").split("/").pop();
  return executable === "tss" || executable === "rtk" || executable === "tss-wrapper.sh";
}

function shellWords(value: string): string[] {
  const input = value.trim();
  const words: string[] = [];
  let current = "";
  let quote = "";
  let escaping = false;

  for (const char of input) {
    if (escaping) {
      current += char;
      escaping = false;
      continue;
    }
    if (char === "\\" && quote !== "'") {
      escaping = true;
      continue;
    }
    if (quote) {
      if (char === quote) quote = "";
      else current += char;
      continue;
    }
    if (char === "'" || char === '"') {
      quote = char;
      continue;
    }
    if (/\s/.test(char)) {
      if (current) {
        words.push(current);
        current = "";
      }
      continue;
    }
    current += char;
  }

  if (current) words.push(current);
  return words;
}
