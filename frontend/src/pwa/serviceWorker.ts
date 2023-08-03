/**
 * @author Sxyntheon
 *
 * @description Service Worker for enabling offline experience with TheSchedule
 **/

importScripts("./cachingStrategies.ts");

const cache: string = "v1";

async function cacheFonts(event: FetchEvent): Promise<Response> {
  return Strategies.CacheFirst(event);
}
async function cacheStyles(event: FetchEvent) {
  return Strategies.StaleWhileRevalidate(event);
}

async function handleFetch(event: FetchEvent): Promise<Response> {
  const url = event.request.url;
  const fileExtension = url.match(/\.[0-9a-z]+$/i);
  if (!fileExtension) {
    return Strategies.NetworkRevalidateAndCache(event);
  } else {
    switch (fileExtension[0]) {
      case ".woff":
        return cacheFonts(event);
      case ".woff2":
        return cacheFonts(event);
      case ".css":
        return cacheStyles(event);
      default:
        return Strategies.NetworkRevalidateAndCache(event);
    }
  }
}
self.addEventListener("fetch", (event: FetchEvent) => {
  event.respondWith(handleFetch(event));
});
