

// --- Electron Window Controls ---
if (window.electronAPI) {
    document.body.classList.add('is-electron');
    document.getElementById('min-btn')?.addEventListener('click', () => {
        window.electronAPI.minimize();
    });
    document.getElementById('max-btn')?.addEventListener('click', () => {
        window.electronAPI.maximize();
    });
    document.getElementById('close-btn')?.addEventListener('click', () => {
        window.electronAPI.close();
    });
}
