(function () {
  // Grant notification permission automatically since the webview
  // may block the native permission prompt.
  if (window.Notification) {
    var _permission = 'granted';
    Object.defineProperty(Notification, 'permission', {
      configurable: true,
      enumerable: true,
      get: function () { return _permission; },
    });
    var _original = Notification.requestPermission;
    Notification.requestPermission = function (callback) {
      var result = Promise.resolve('granted');
      if (callback) { callback('granted'); }
      return result;
    };
  }

  if (window.__TAURI__ === undefined) {
    console.warn('rsStoat bridge: __TAURI__ not available, native features disabled');
    return;
  }

  // Override getDisplayMedia to return the stream captured by our
  // onceScreenPicker impl, since Tauri v2.11 lacks setDisplayMediaRequestHandler.
  var _origGetDisplayMedia = navigator.mediaDevices.getDisplayMedia.bind(navigator.mediaDevices);
  navigator.mediaDevices.getDisplayMedia = function () {
    if (window.native && window.native.__screenShareStream) {
      var stream = window.native.__screenShareStream;
      window.native.__screenShareStream = null;
      return Promise.resolve(stream);
    }
    return _origGetDisplayMedia.apply(this, arguments);
  };

  var invoke = window.__TAURI__.core.invoke;

  // ---- window.native API ----
  if (!window.native) {
    window.native = {};
  }

  window.native.versions = {
    node: '',
    chrome: '',
    tauri: '2',
    desktop: '1.4.0',
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

  window.native.onceScreenPicker = function (callback) {
    // Tauri v2.11 lacks setDisplayMediaRequestHandler (coming in v2.12).
    // Use the native getDisplayMedia() picker instead.
    navigator.mediaDevices.getDisplayMedia({ video: true, audio: true })
      .then(function (stream) {
        window.native.__screenShareStream = stream;
        callback([{ idx: 0, name: 'Screen / Window', isFullScreen: true }]);
      })
      .catch(function () {
        callback([]);
      });
  };

  window.native.screenPickerCallback = function () {
    // Stream was already captured by the native picker.
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
