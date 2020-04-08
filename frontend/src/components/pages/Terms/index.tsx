import React from "react";

import TermsTemplate from "../../templates/Terms";

const Terms: React.FC = () => {
  const owner = process.env.ARTERIA_OWNER as string;
  return <TermsTemplate owner={owner} />;
};

export default Terms;
