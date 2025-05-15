import {MenuProps, SelectProps} from "antd";

export type MenuItem = Required<MenuProps>["items"][number];

export type SelectOption = NonNullable<SelectProps["options"]>[number];
