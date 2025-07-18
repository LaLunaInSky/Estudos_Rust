<div
    onkeydowncapture={(e) => alert(`<div> ${e.key}`)}
    role="presentation"
    class="
        mb-2 mt-4 bg-slate-700/70 *:text-center *:text-neutral-50 border-2 border-slate-50/40 *:py-1 rounded-lg
    "
>
    <input
        onkeydowncapture={(e) => alert(`<input> ${e.key}`)}
    />
</div>