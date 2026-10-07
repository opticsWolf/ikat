const assembly = document.querySelector("#assembly-board");
const assembleButton = document.querySelector("#assemble-button");
const assemblyStatus = document.querySelector("#assembly-status");

if (assembly && assembleButton) {
  assembleButton.addEventListener("click", () => {
    const isAssembled = assembly.dataset.state !== "assembled";
    assembly.dataset.state = isAssembled ? "assembled" : "exploded";
    assembleButton.setAttribute("aria-pressed", String(isAssembled));
    assembleButton.textContent = isAssembled ? "Explode the view" : "Assemble the layers";
    assemblyStatus.textContent = isAssembled
      ? "The inputs are aligned with the emitted TeX and PDF stage."
      : "The parts are separated so each input and output is visible.";
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
