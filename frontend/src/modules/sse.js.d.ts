declare module "sse.js" {
  interface EventSourceInitExtend {
    payload: any;
  }

  interface SSE {
    prototype: EventSource;
    new (url: string, eventSourceInitDict?: EventSourceInitExtend): EventSource;
    readonly CLOSED: number;
    readonly CONNECTING: number;
    readonly OPEN: number;
  }

  const SSE: SSE;

  export { SSE };
}
