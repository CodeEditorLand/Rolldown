x = () => [
	tag` + "`x`" + `,
	tag` + "`\\xFF`" + `,
	tag` + "`\\x`" + `,
	tag` + "`\\u`" + `,
]
y = () => [
	tag` + "`x${y}z`" + `,
	tag` + "`\\xFF${y}z`" + `,
	tag` + "`x${y}\\z`" + `,
	tag` + "`x${y}\\u`" + `,
]