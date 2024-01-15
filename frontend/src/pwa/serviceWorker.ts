/**
 * @author Sxyntheon
 *
 * @description Service Worker for enabling offline experience with TheSchedule
**/
import { Strategies }  from "./cachingStrategies"

const cache: string = "v1";


async function cacheFonts(event: FetchEvent): Promise<Response> {
  return Strategies.CacheFirst(event);
}
async function cacheStyles(event: FetchEvent): Promise<Response> {
  return Strategies.StaleWhileRevalidate(event);
}
async function cacheScripts(event: FetchEvent): Promise<Response> {
  return Strategies.StaleWhileRevalidate(event);
}
async function cacheComponents(event: FetchEvent): Promise<Response> {
  return Strategies.StaleWhileRevalidate(event)
}
async function handleFetch(event: FetchEvent): Promise<Response> {
  const url = event.request.url;
  const fileExtension = url.match(/\.[0-9a-z]+$/i);
  const blacklist = "https://localhost:8080";
  if (url.startsWith(blacklist)) {
    console.log(url, " is blacklisted")
    return Strategies.NetworkOnly(event)
  }
  if (!fileExtension) {
    return Strategies.NetworkRevalidateAndCache(event);
  } else {
    switch (fileExtension[0]) {
      case ".woff":
        console.log(url)
        console.count("Font from Cache")
        return cacheFonts(event);
      case ".woff2":
        console.log(url);
        console.count("Font from Cache");
        return cacheFonts(event);
      case ".css":
        console.log(url);
        console.count("css from cache")
        return cacheStyles(event);
      case ".scss":
        console.log(url);
        console.count("css from cache")
        return cacheStyles(event)
      case ".ts":
        console.log(url);
        console.count("script from cache")
        return cacheScripts(event)
      case ".tsx":
        console.log(url);
        console.count("component from cache")
        return cacheComponents(event)
      default:
        console.log(url);
        console.count("default caching")
        return Strategies.NetworkRevalidateAndCache(event);
    }
  }
}
self.addEventListener("fetch", (event: FetchEvent) => {
  event.respondWith(handleFetch(event));
});