export const TssPlugin = async () => {
  return {
    "tool.execute.before": async (input, output) => {
      if (!input || input.tool !== "bash") return
      if (!output || !output.args || typeof output.args.command !== "string") return

      const command = output.args.command.trim()
      if (!command || command.startsWith("tss ") || command.startsWith("env TSS_AGENT=")) return

      output.args.command = `env TSS_AGENT=opencode tss run -- bash -lc ${shellQuote(output.args.command)}`
    },
  }
}

function shellQuote(value) {
  return `'${String(value).replace(/'/g, `'\"'\"'`)}'`
}
