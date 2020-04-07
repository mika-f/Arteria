import React from "react";

import HyperLink from "../../atoms/HyperLink";
import RouterLink from "../../atoms/RouterLink";

type Props = {
  className?: string;
  children?: any;
  href: string;
};

const Link: React.FC<Props> = ({ className, children, href }) => {
  if (href.startsWith("/")) {
    return (
      <RouterLink to={href} className={className}>
        {children}
      </RouterLink>
    );
  }

  return (
    <HyperLink href={href} className={className} target="_blank" rel="noreferrer noopener">
      {children}
    </HyperLink>
  );
};

export default Link;
