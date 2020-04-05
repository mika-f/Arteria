import styled from "styled-components";

const Button = styled.button`
  padding: 12px;
  font-size: 16px;
  cursor: pointer;
  border: 0;

  :focus {
    outline: 0;
  }
`;

const PrimaryButton = styled(Button)`
  color: #fff;
  background-color: #0095ff;

  :disabled {
    color: #ccc;
    background-color: #004a83;
  }
`;

const LinkButton = styled(Button)`
  padding: 0;
  font-size: 16px;
  color: inherit;
  background-color: transparent;
`;

export { Button, PrimaryButton, LinkButton };
