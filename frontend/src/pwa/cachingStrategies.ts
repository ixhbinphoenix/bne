export class Strategies {
  static getTimeStampStatus(name: string, maxAgeSeconds: number = 2630000): boolean {
    const cookie = document.cookie.match("(^|;)\\s*" + name + "\\s*=\\s*([^;]+)")?.pop() || "";
    if (!cookie) {
      document.cookie = `${name}; max-age=${maxAgeSeconds}; secure; samesite=strict`;
      return false;
    }
    return true;
  }
  static async clearCache(cacheName: string): Promise<boolean> {
    return await caches.delete(cacheName);
  }
  public static async CacheOnly(event: FetchEvent): Promise<Response> {
    return (await caches.match(event.request)) || this.ErrorMessage();
  }
  public static async CacheFirst(event: FetchEvent, cacheName: string = "generic"): Promise<Response> {
    //const notExpired = this.getTimeStampStatus(cacheName);
    const response = await caches.match(event.request);
    return (
      response ||
      fetch(event.request).then(async (response): Promise<Response> => {
        const cache = await caches.open(cacheName);
        cache.put(event.request, response.clone());
        return response;
      })
    );
  }
  public static async NetworkOnly(event: FetchEvent): Promise<Response> {
    return fetch(event.request);
  }
  public static async NetworkFirst(event: FetchEvent): Promise<Response> {
    return fetch(event.request).then(async (response: Response) => {
      if (response.ok) {
        return response;
      } else {
        return (await caches.match(event.request)) || this.ErrorMessage();
      }
    });
  }
  public static async StaleWhileRevalidate(event: FetchEvent, cacheName: string = "generic"): Promise<Response> {
    return caches.match(event.request).then((cacheResponse: Response | undefined) => {
      let fetchResponse = fetch(event.request).then(async (response): Promise<Response> => {
        const cache = await caches.open(cacheName);
        cache.put(event.request, response.clone());
        return response;
      });
      return cacheResponse || fetchResponse || this.ErrorMessage();
    });
  }
  public static async NetworkRevalidateAndCache(event: FetchEvent, cacheName: string = "generic"): Promise<Response> {
    return fetch(event.request).then(
      async (fetchResponse: Response): Promise<Response> => {
        if (fetchResponse.ok) {
          const cache = await caches.open(cacheName);
          cache.put(event.request, fetchResponse.clone());
          return fetchResponse;
        }
        const cacheResponse = await caches.match(event.request);
        return cacheResponse || this.ErrorMessage();
      },
      async () => {
        const cacheResponse = await caches.match(event.request);
        return cacheResponse || this.ErrorMessage();
      }
    );
  }
  public static async ErrorMessage(): Promise<Response> {
    console.log("request was not in cache");
    return new Response(JSON.stringify({ success: false, message: "Connection offline" }), { status: 404 });
  }
}
