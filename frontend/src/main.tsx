import React from "react";
import ReactDOM from "react-dom/client";
import "./index.css";
import Login from "./views/login.tsx";
import { ROUTER } from "./routes.tsx";

type RouterProps = {};
type RouterState = {
    path: Array<string>;
};

class Router extends React.Component<RouterProps, RouterState> {
    constructor(props: RouterProps) {
        super(props);

        this.state = {
            path: [],
        };
    }

    componentDidMount() {
        // Update state to match url
        const setPath = () => {
            const rawPath = window.location.hash;

            // Ensure well-formed path i.e. always have a #/
            if (!rawPath.startsWith("#/")) {
                window.location.hash = "#/";

                // this method will be immediately triggered again
                return;
            }

            // Split everything after #/
            const path = rawPath.substring(2).split("/");

            // #/ should result in [] not [""]
            if (path.length === 1 && path[0] === "") {
                path.shift();
            }

            this.setState({ path });
        };

        setPath();
        window.addEventListener("hashchange", setPath);
    }

    render() {
        const { path } = this.state;

        if (path[0] && path[0] === "login") {
            return <Login />;
        }

        return <div className="content-container">{ROUTER.matchAndRender(path) || <div>Unknown route</div>}</div>;
    }
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <>
        <Router />
    </>
);
