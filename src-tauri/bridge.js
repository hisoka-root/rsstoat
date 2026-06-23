(function () {
  if (window.__TAURI__ === undefined) {
    console.warn('rsStoat bridge: __TAURI__ not available, native features disabled');
    return;
  }

  var invoke = window.__TAURI__.core.invoke;

  // ---- window.native API ----
  if (!window.native) {
    window.native = {};
  }

  window.native.versions = {
    node: '',
    chrome: '',
    tauri: '2',
    desktop: '1.3.0',
  };

  window.native.minimise = function () {
    invoke('minimise');
  };

  window.native.maximise = function () {
    invoke('maximise');
  };

  window.native.close = function () {
    invoke('close_window');
  };

  window.native.setBadgeCount = function (count) {
    invoke('set_badge_count', { count: count });
  };

  // ---- window.desktopConfig API ----
  if (!window.desktopConfig) {
    window.desktopConfig = {};
  }

  window.desktopConfig.get = async function () {
    return invoke('get_config');
  };

  window.desktopConfig.set = async function (config) {
    return invoke('set_config', { newConfig: config });
  };

  window.desktopConfig.getAutostart = async function () {
    return invoke('get_autostart');
  };

  window.desktopConfig.setAutostart = async function (value) {
    return invoke('set_autostart', { enabled: value });
  };

  // ---- Zoom shortcuts ----
  (function () {
    var zoom = 1;
    document.addEventListener('keydown', function (e) {
      if (e.ctrlKey && (e.key === '=' || e.key === '+')) {
        e.preventDefault();
        zoom = Math.min(zoom + 0.1, 5);
        document.body.style.zoom = zoom;
      } else if (e.ctrlKey && e.key === '-') {
        e.preventDefault();
        zoom = Math.max(zoom - 0.1, 0.2);
        document.body.style.zoom = zoom;
      } else if (e.ctrlKey && e.key === '0') {
        e.preventDefault();
        zoom = 1;
        document.body.style.zoom = '';
      } else if (e.key === 'F5' || ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'r')) {
        e.preventDefault();
        location.reload();
      }
    });
  })();

  // ---- Keyboard shortcut: Escape from fullscreen ----
  document.addEventListener('keydown', function (e) {
    if (e.key === 'Escape' && document.fullscreenElement) {
      document.exitFullscreen();
    }
  });
})();
