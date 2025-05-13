import React, {useState} from "react";
import {Modal} from "antd";
import Draggable from "react-draggable";
import {useDraggable} from "./use-draggable.tsx";
import {DraggableModalProps} from "@src/components/common/draggable-modal/types.ts";

export const DraggableModal: React.FC<DraggableModalProps> = (props) => {
    const [disabled, setDisabled] = useState(true);
    const {draggableRef, bounds, onStart} = useDraggable();
    return (
        <Modal {...props}
               className={"draggable-modal"}
               title={
                   <div style={{width: "100%", cursor: "move"}}
                        onBlur={() => {
                        }}
                        onFocus={() => {
                        }}
                        onMouseOver={() => disabled && setDisabled(false)}
                        onMouseOut={() => setDisabled(true)}>
                       {props.title}
                   </div>}
               modalRender={(modal) => (
                   <Draggable disabled={disabled}
                              bounds={bounds}
                              nodeRef={draggableRef}
                              onStart={(event, uiData) => onStart(event, uiData)}>
                       <div ref={draggableRef}>
                           {modal}
                       </div>
                   </Draggable>
               )}>
            {props.children}
        </Modal>
    )
}