import React from "react";
import Helmet from "react-helmet";

import AboutTemplate from "../../templates/About";

const About: React.FC = () => {
  return (
    <>
      <Helmet>
        <title>Arteria について</title>
      </Helmet>
      <AboutTemplate />
    </>
  );
};

export default About;
