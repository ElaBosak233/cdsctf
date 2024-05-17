export default function MDIcon({
	size,
	children,
}: {
	size?: number;
	children: React.ReactNode;
}) {
	return (
		<span
			className="material-symbols-rounded"
			style={{
				fontSize: size || 24,
			}}
		>
			{children}
		</span>
	);
}
