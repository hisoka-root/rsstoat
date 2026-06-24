(function () {
  if (window.__TAURI__ === undefined) {
    console.warn('rsStoat bridge: __TAURI__ not available, native features disabled');
    return;
  }

  var invoke = window.__TAURI__.core.invoke;

  // Grant notification permission and route through Tauri plugin
  // for native desktop notifications instead of in-webview popups.
  if (window.Notification) {
    var _OrigNotification = window.Notification;
    // Replace constructor to send native notifications via Tauri plugin
    window.Notification = function (title, options) {
      invoke('plugin:notification|notify', {
        title: title,
        body: (options && options.body) || '',
      });
    };
    window.Notification.prototype = _OrigNotification.prototype;
    // Auto-grant permission so the web app proceeds
    Object.defineProperty(window.Notification, 'permission', {
      configurable: true,
      enumerable: true,
      get: function () { return 'granted'; },
    });
    window.Notification.requestPermission = function (callback) {
      var result = Promise.resolve('granted');
      if (callback) { callback('granted'); }
      return result;
    };
  }

  // ---- window.native API ----
  if (!window.native) {
    window.native = {};
  }

  window.native.versions = {
    node: function () { return ''; },
    chrome: function () { return ''; },
    electron: function () { return ''; },
    desktop: function () { return '1.4.2'; },
  };

  window.native.minimise = function () {
    invoke('minimise');
  };

  window.native.checkUpdate = function () {
    return invoke('check_update');
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

  window.native.onceScreenPicker = function (callback) {
    // Tauri v2.11 lacks setDisplayMediaRequestHandler (coming in v2.12).
    // Provide a single dummy source so the web app shows the picker.
    // When the user selects it, screenPickerCallback fires (no-op) and
    // the web app calls getDisplayMedia() which shows the native picker.
    callback([{ idx: 0, name: 'Screen / Window', isFullScreen: true }]);
  };

  window.native.screenPickerCallback = function (idx, audio) {
    // Native browser picker handles the stream; no-op here.
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
      } else if (e.key === 'F12') {
        e.preventDefault();
        invoke('open_devtools');
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
