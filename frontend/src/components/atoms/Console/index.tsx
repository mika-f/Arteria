import React from "react";
import styled from "styled-components";

type Props = {
  lines: string[];
};

const Outer = styled.pre`
  max-width: calc(100% - 2px);
  padding: 8px 16px;
  font-size: 16px;
  line-height: 1.25;
  color: white;
  overflow-x: auto;
  background-color: #252526;
  border: 1px solid #bbb;
`;

const StdOut = styled.span`
  display: block;
  font-family: Consolas, Menlo, Monaco, "Courier New", monospace;
`;

const StdErr = styled.span`
  display: block;
  font-family: Consolas, Menlo, Monaco, "Courier New", monospace;
  background: #f88070;
`;

const Console: React.FC<Props> = ({ lines }) => {
  return (
    <Outer>
      {lines.map(w => {
        if (w.startsWith("stdout: ")) {
          return <StdOut>{w.substring("stdout: ".length)}</StdOut>;
        }
        if (w.startsWith("stderr: ")) {
          return <StdErr>{w.substring("stderr: ".length)}</StdErr>;
        }
        return <StdOut>{w}</StdOut>;
      })}
    </Outer>
  );
};

export default Console;
