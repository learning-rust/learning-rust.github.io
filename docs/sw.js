const cacheName = 'learning-rust-{{ now.Format "2006-01-02" }}';
const staticAssets = [
    './',
    './index.html',
    './manifest.json',
    './docs/**/*',
    './favicon/android-chrome-192x192.png',
    './favicon/android-chrome-512x512.png',
    './favicon/apple-touch-icon.png',
    './favicon/favicon.ico',
    './favicon/favicon-16x16.png',
    './favicon/favicon-32x32.png',
    './css/home.min.*.css',
    './css/docs.min.*.css',
    './js/home.min.*.js',
    './js/docs.min.*.js',
];

self.addEventListener('install', async e => {
    const cache = await caches.open(cacheName);
    await cache.addAll(staticAssets);
    return self.skipWaiting();
});

self.addEventListener('activate', e => {
    self.clients.claim();
});

self.addEventListener('fetch', async e => {
    const req = e.request;
    const url = new URL(req.url);

    if (url.origin === location.origin) {
        e.respondWith(cacheFirst(req));
    } else {
        e.respondWith(networkFirst(req));
    }
});

async function cacheFirst(req) {
    const cache = await caches.open(cacheName);
    const cached = await cache.match(req);
    return cached || fetch(req);
}

async function networkFirst(req) {
    const cache = await caches.open(cacheName);
    try {
        const fresh = await fetch(req);
        cache.put(req, fresh.clone());
        return fresh;
    } catch (e) {
        const cached = await cache.match(req);
        return cached;
    }
}