import type { CSSProperties, FC, MouseEvent, PropsWithChildren, ReactElement } from 'react';
export type ButtonProps = PropsWithChildren<{
    className?: string;
    disabled?: boolean;
    endIcon?: ReactElement;
    onClick?: (e: MouseEvent<HTMLButtonElement>) => void;
    startIcon?: ReactElement;
    style?: CSSProperties;
    tabIndex?: number;
}>;
export declare const Button: FC<ButtonProps>;
