import "solid-js";

declare module "solid-js" {
  namespace JSX {
    // eslint-disable-next-line @typescript-eslint/consistent-type-definitions
    interface IntrinsicElements {
      "ion-icon": HTMLAttributes<HTMLElement> & {
        name?: string
      }
    }
  }
}
