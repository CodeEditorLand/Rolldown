// Helper functions for "super" shouldn't be inserted into this outer function
export default (async function () {
	class y extends z {
		foo = async () => super.foo = 'foo'
	}
	await new y().foo()()
})()