import { forwardRef, ReactNode } from "react";
import { ThemeIcon, ThemeIconProps } from "@mantine/core";

interface MDIconProps extends ThemeIconProps {
	children: ReactNode;
}

const MDIcon = forwardRef<HTMLSpanElement, MDIconProps>(
	({ children, size, ...themeIconProps }, ref) => {
		return (
			<ThemeIcon variant="transparent" {...themeIconProps} size={size}>
				<span
					ref={ref}
					className="material-symbols-rounded"
					style={{
						fontSize: size,
					}}
				>
					{children}
				</span>
			</ThemeIcon>
		);
	}
);

export default MDIcon;
