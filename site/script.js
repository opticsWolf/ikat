const assembly = document.querySelector("#assembly-board");
const assembleButton = document.querySelector("#assemble-button");
const assemblyStatus = document.querySelector("#assembly-status");

if (assembly && assembleButton) {
  const outputPart = assembly.querySelector(".part-output");
  const leaderLines = [...assembly.querySelectorAll(".leader")];
  let connectorFrame = 0;
  let connectorTimer = 0;

  const updateConnectors = () => {
    const boardRect = assembly.getBoundingClientRect();
    const source = assembly.querySelector(".part-source").getBoundingClientRect();
    const figures = assembly.querySelector(".part-figures").getBoundingClientRect();
    const engine = assembly.querySelector(".part-engine").getBoundingClientRect();
    const output = outputPart.getBoundingClientRect();
    const port = (rect, x, y) => ({ x: rect.left + rect.width * x, y: rect.top + rect.height * y });
    const mobile = window.matchMedia("(max-width: 620px)").matches;
    const links = mobile
      ? [
          [port(source, 0.5, 1), port(figures, 0.5, 0)],
          [port(figures, 0.5, 1), port(engine, 0.5, 0)],
          [port(engine, 0.5, 1), port(output, 0.5, 0)],
        ]
      : [
          [port(source, 0.5, 1), port(engine, 0.5, 0)],
          [port(figures, 0, 1), port(engine, 1, 0)],
          [port(engine, 1, 0.5), port(output, 0, 0.5)],
        ];

    links.forEach(([start, end], index) => {
      const line = leaderLines[index];
      const dx = end.x - start.x;
      const dy = end.y - start.y;
      line.style.left = `${start.x - boardRect.left}px`;
      line.style.top = `${start.y - boardRect.top}px`;
      line.style.width = `${Math.hypot(dx, dy)}px`;
      line.style.height = "1px";
      line.style.transform = `rotate(${Math.atan2(dy, dx)}rad)`;
    });
  };

  const stopFollowing = () => {
    window.cancelAnimationFrame(connectorFrame);
    window.clearTimeout(connectorTimer);
    connectorFrame = 0;
    connectorTimer = 0;
    updateConnectors();
  };

  const followParts = () => {
    window.cancelAnimationFrame(connectorFrame);
    window.clearTimeout(connectorTimer);
    const follow = () => {
      updateConnectors();
      connectorFrame = window.requestAnimationFrame(follow);
    };
    connectorFrame = window.requestAnimationFrame(follow);
    connectorTimer = window.setTimeout(stopFollowing, 1400);
  };

  updateConnectors();
  window.addEventListener("resize", updateConnectors, { passive: true });

  outputPart?.addEventListener("transitionend", (event) => {
    if (event.propertyName !== "top") return;
    stopFollowing();
    assemblyStatus.textContent = assembly.dataset.state === "assembled"
      ? "Manuscript and figure inputs connect through the Rust core to the document output."
      : "The parts are separated so each input and output is visible.";
  });

  assembleButton.addEventListener("click", () => {
    const isAssembled = assembly.dataset.state !== "assembled";
    assembly.dataset.state = isAssembled ? "assembled" : "exploded";
    assembly.setAttribute("aria-label", isAssembled
      ? "Assembled view of ikat’s document pipeline"
      : "Exploded view of ikat’s document pipeline");
    assembleButton.setAttribute("aria-pressed", String(isAssembled));
    assembleButton.textContent = isAssembled ? "Explode the view" : "Assemble the layers";
    assemblyStatus.textContent = isAssembled
      ? "Aligning the manuscript and figure inputs with the Rust core…"
      : "Separating the source plates so each pipeline stage is visible…";
    followParts();
  });
}

const copyStatus = document.querySelector("#copy-status");
const copyButtons = document.querySelectorAll("[data-copy]");

async function copyText(text) {
  if (navigator.clipboard && window.isSecureContext) {
    await navigator.clipboard.writeText(text);
    return;
  }

  const temporary = document.createElement("textarea");
  temporary.value = text;
  temporary.setAttribute("readonly", "");
  temporary.style.position = "fixed";
  temporary.style.opacity = "0";
  document.body.append(temporary);
  temporary.select();
  const copied = document.execCommand("copy");
  temporary.remove();
  if (!copied) throw new Error("Clipboard access is unavailable");
}

copyButtons.forEach((button) => {
  button.addEventListener("click", async () => {
    const source = document.getElementById(button.dataset.copy);
    if (!source) {
      copyStatus.textContent = "Could not find the command block. Select and copy it manually.";
      return;
    }

    try {
      await copyText(source.textContent.trim());
      button.textContent = "Copied";
      copyStatus.textContent = "Commands copied. Paste them into your terminal.";
      window.setTimeout(() => {
        button.textContent = "Copy commands";
      }, 1800);
    } catch {
      copyStatus.textContent = "Clipboard access was blocked. Select the commands and copy them manually.";
    }
  });
});
