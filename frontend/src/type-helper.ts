import { StyledComponent } from "styled-components";

export type ExtractProps<T> =
  // prettier-ignore
  T extends StyledComponent<any, any, infer R, never> ? R
  : T extends React.FunctionComponent<infer R> ? R
  : string;
