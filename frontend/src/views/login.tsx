import React from "react";
import Input from "../components/input.tsx";
import { Api } from "../api/api.ts";
import { toast } from "react-toastify";
import { okOrToast } from "../utils/helper";

type LoginProps = {
    onLogin(): void;
};
type LoginState = {
    username: string;
    password: string;
};

export default class Login extends React.Component<LoginProps, LoginState> {
    state: LoginState = {
        username: "",
        password: "",
    };

    performLogin() {
        Api.auth.login(this.state.username, this.state.password).then(
            okOrToast(() => {
                toast.success("Authenticated successfully");
                this.props.onLogin();
            })
        );
    }

    render() {
        return (
            <div className={"login-container"}>
                <form
                    method={"post"}
                    className={"login"}
                    onSubmit={(e) => {
                        e.preventDefault();
                        this.performLogin();
                    }}
                >
                    <Input
                        value={this.state.username}
                        onChange={(v: string) => {
                            this.setState({ username: v });
                        }}
                    />
                    <Input
                        value={this.state.password}
                        onChange={(v: string) => {
                            this.setState({ password: v });
                        }}
                        type={"password"}
                    />
                </form>
            </div>
        );
    }
}
