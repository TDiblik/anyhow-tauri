const { invoke } = window.__TAURI__.tauri;

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#test-1").addEventListener("click", async () => {
    console.log(await invoke("test"));
  });
  document.querySelector("#test-2").addEventListener("click", async () => {
    console.log(await invoke("test_anyhow_success"));
  });
  document.querySelector("#test-3").addEventListener("click", async () => {
    console.log(await invoke("test_throw"));
  });
  document.querySelector("#test-4").addEventListener("click", async () => {
    console.log(await invoke("test_pure_err_conversion"));
  });
  document.querySelector("#test-5").addEventListener("click", async () => {
    console.log(await invoke("test_bail"));
  });
  document.querySelector("#test-6").addEventListener("click", async () => {
    console.log(await invoke("test_ensure"));
  });
});
