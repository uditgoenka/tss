import { definePluginEntry } from "openclaw/plugin-sdk/plugin-entry";

function shellQuote(value) {
  return `'${String(value).replace(/'/g, `'\"'\"'`)}'`;
}

function wrap(command) {
  const trimmed = String(command).trim();
  if (
    !trimmed ||
    alreadyOwned(trimmed) ||
    trimmed.startsWith("env TSS_AGENT=") ||
    trimmed.startsWith("TSS_AGENT=") ||
    trimmed.startsWith("TSS_BYPASS=1") ||
    trimmed.startsWith("env TSS_BYPASS=1")
  ) return command;
  return `env TSS_AGENT=openclaw tss run -- bash -lc ${shellQuote(command)}`;
}

function alreadyOwned(command) {
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

function shellWords(value) {
  const input = String(value).trim();
  const words = [];
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

export default definePluginEntry({
  id: "tss",
  name: "TSS",
  register(api) {
    api.on("before_tool_call", async (event) => {
      const command = event?.params?.command;
      if (typeof command !== "string") return {};

      return {
        params: {
          ...event.params,
          command: wrap(command),
        },
      };
    });
  },
});
