import React from "react";
import styled from "styled-components";

type RowProps = {
  className?: string;
  children?: any;
  lg: number;
  md: number;
  sm: number;
};

type GridProps = {
  className?: string;
  children?: any;
  rows: number;
};

const GridContext = React.createContext(12);

const GridInternal = styled.div`
  display: flex;
  flex-flow: row wrap;
`;

const Grid: React.FC<GridProps> = ({ children, className, rows }) => {
  return (
    <GridContext.Provider value={rows}>
      <GridInternal className={className}>{children}</GridInternal>
    </GridContext.Provider>
  );
};

const RowInternal = styled.div<RowProps & { rows: number }>`
  min-width: 0;

  @media screen and (max-width: 576px) {
    flex: 1 1 ${props => (props.sm / props.rows) * 100}%;
  }

  @media screen and (min-width: 576px) {
    flex: 1 1 ${props => (props.md / props.rows) * 100}%;
  }

  @media screen and (min-width: 768px) {
    flex: 1 1 ${props => (props.lg / props.rows) * 100}%;
  }
`;

const Row: React.FC<RowProps> = ({ children, className, sm, md, lg }) => {
  return (
    <GridContext.Consumer>
      {value => (
        <RowInternal className={className} lg={lg} md={md} sm={sm} rows={value}>
          {children}
        </RowInternal>
      )}
    </GridContext.Consumer>
  );
};

export { Grid, Row };
