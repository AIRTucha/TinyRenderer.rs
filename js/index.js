window.onload = () => {
    const diffuse = document.createElement("img");
    const nm = document.createElement("img");
    const spec = document.createElement("img");

    diffuse.src = "obj/african_head/african_head_diffuse.jpg";
    diffuse.id = "diffuse";
    diffuse.hidden = true;

    nm.src = "obj/african_head/african_head_nm.jpg";
    nm.id = "nm";
    nm.hidden = true;

    spec.src = "obj/african_head/african_head_spec.jpg";
    spec.id = "spec";
    spec.hidden = true;

    document.body.appendChild(diffuse);
    document.body.appendChild(nm);
    document.body.appendChild(spec);

    diffuse.onload = () => {
        nm.onload = () => {
            spec.onload = () => {
                console.log("ok");
                import("../pkg/index.js").catch(console.error);
            }
        }
    }
}

