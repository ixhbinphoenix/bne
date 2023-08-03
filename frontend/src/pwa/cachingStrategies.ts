class Strategies {
  public static cacheName = "v1";

  public static async CacheOnly(event: FetchEvent): Promise<Response | undefined> {
    return caches.match(event.request);
  }
  public static async CacheFirst(event: FetchEvent): Promise<Response> {
    const response = await caches.match(event.request);
    return (
      response ||
      fetch(event.request).then(async (response): Promise<Response> => {
        const cache = await caches.open(Strategies.cacheName);
        cache.put(event.request, response.clone());
        return response;
      })
    );
  }
  public static async NetworkOnly(event: FetchEvent): Promise<Response> {
    return fetch(event.request);
  }
  public static async NetworkFirst(event: FetchEvent): Promise<Response | undefined> {
    return fetch(event.request).then((response: Response) => {
      if (response.ok) {
        return response;
      } else {
        return caches.match(event.request);
      }
    });
  }
  public static async StaleWhileRevalidate(event: FetchEvent): Promise<Response> {
    return caches.match(event.request).then((cacheResponse: Response | undefined) => {
      let fetchResponse = fetch(event.request).then(async (response): Promise<Response> => {
        const cache = await caches.open(Strategies.cacheName);
        cache.put(event.request, response.clone());
        return response;
      });
      return cacheResponse || fetchResponse;
    });
  }
  public static async NetworkRevalidateAndCache(event: FetchEvent): Promise<Response> {
    return fetch(event.request).then(
      async (fetchResponse: Response): Promise<Response> => {
        if (fetchResponse.ok) {
          const cache = await caches.open(Strategies.cacheName);
          cache.put(event.request, fetchResponse.clone());
          return fetchResponse;
        }
        const cacheResponse = await caches.match(event.request);
        return (
          cacheResponse ||
          new Response(JSON.stringify({ success: false, message: "Connection offline" }), { status: 404 })
        );
      },
      async () => {
        const cacheResponse = await caches.match(event.request);
        return (
          cacheResponse ||
          new Response(JSON.stringify({ success: false, message: "Connection offline" }), { status: 404 })
        );
      }
    );
  }
}
