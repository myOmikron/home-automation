import React from "react";
import Input from "../components/input.tsx";

type LoginProps = {};
type LoginState = {
    username: string;
    password: string;
};

export default class Login extends React.Component<LoginProps, LoginState> {
    constructor(props: LoginProps) {
        super(props);

        this.state = {
            username: "",
            password: "",
        };
    }

    render() {
        return (
            <div className={"login-container"}>
                <div className={"login"}>
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
                </div>
            </div>
        );
    }
}
