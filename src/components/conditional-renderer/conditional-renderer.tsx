import React from "react";

interface ConditionalRendererProps {
    isVisible: boolean;
    childrenComponent: React.ReactNode;
}

export const ConditionalRenderer: React.FC<ConditionalRendererProps> = ({isVisible, childrenComponent}) => {
    if (!isVisible) {
        return null;
    }

    return <>{childrenComponent}</>;
};

export default ConditionalRenderer;