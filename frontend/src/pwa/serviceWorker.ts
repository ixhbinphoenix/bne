/**
 * @author Sxyntheon
 *
 * @description Service Worker for enabling offline experience with TheSchedule
 **/
import { Strategies } from "./cachingStrategies";

const cache: string = "v1";

async function cacheFonts(event: FetchEvent): Promise<Response> {
  return Strategies.CacheFirst(event);
}
async function cacheStyles(event: FetchEvent): Promise<Response> {
  return Strategies.CacheFirst(event);
}
async function cacheScripts(event: FetchEvent): Promise<Response> {
  return Strategies.CacheFirst(event);
}
async function cacheComponents(event: FetchEvent): Promise<Response> {
  return Strategies.StaleWhileRevalidate(event);
}
async function handleFetch(event: FetchEvent): Promise<Response> {
  const url = new URL(event.request.url);
  const fileExtension = url.href.match(/\.[0-9a-z]+$/i);
  const backend_url = "https://api.theschedule.de";
  const untis_domain = "https://borys.webuntis.com";
  if (url.href.startsWith(untis_domain) || event.request.method != "GET") {
    return Strategies.NetworkOnly(event);
  }
  if (url.href.startsWith(backend_url)) {
    console.log(url, " is blacklisted");

    switch (url.pathname) {
      case "/get_timetable":
        console.log("Timetable is network first");
        return Strategies.StaleWhileRevalidate(event);
      case "/get_lernbueros":
        console.log("LBs is Cache first");
        return Strategies.StaleWhileRevalidate(event);
      case "/get_free_rooms":
        console.log("Free Rooms is Cache first");
        return Strategies.StaleWhileRevalidate(event);
    }
    return Strategies.NetworkOnly(event);
  }
  if (!fileExtension) {
    return Strategies.NetworkRevalidateAndCache(event);
  } else {
    switch (fileExtension[0]) {
      case ".woff2":
        console.log(url);
        console.count("Font from Cache");
        return cacheFonts(event);
      case ".css":
        console.log(url);
        console.count("css from cache");
        return cacheStyles(event);
      case ".js":
        console.log(url);
        console.count("Script from Cache");
        return cacheScripts(event);
      case ".svg":
        return cacheStyles(event);
      default:
        console.log(url);
        console.count("default caching");
        return Strategies.NetworkOnly(event);
    }
  }
}
self.addEventListener("fetch", (event: FetchEvent) => {
  event.respondWith(handleFetch(event));
});
