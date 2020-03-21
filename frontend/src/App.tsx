import React from "react";
import { Helmet } from "react-helmet";
import { BrowserRouter, Route } from "react-router-dom";

import Root from "./components/pages/Root";

const App: React.FC = () => {
  return (
    <>
      <Helmet defaultTitle="Arteria" titleTemplate="%s | Arteria" />
      <BrowserRouter>
        <Route path="/" component={Root} exact />
      </BrowserRouter>
    </>
  );
};

export default App;