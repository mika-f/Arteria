import React from "react";
import Helmet from "react-helmet";

import TermsTemplate from "../../templates/Terms";

const Terms: React.FC = () => {
  const owner = process.env.ARTERIA_OWNER as string;
  return (
    <>
      <Helmet>
        <title>利用規約</title>
      </Helmet>
      <TermsTemplate owner={owner} />
    </>
  );
};

export default Terms;
