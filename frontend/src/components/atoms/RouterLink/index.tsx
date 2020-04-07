import { Link } from "react-router-dom";
import styled from "styled-components";

const RouterLink = styled(Link)`
  color: #ccc;
  text-decoration: none;
  cursor: pointer;

  :hover {
    text-decoration: underline;
  }
`;

export default RouterLink;
