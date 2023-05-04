import React from "react";
import { GetMeResponse } from "../api/generated";
import { Api } from "../api/api";
import Loading from "../components/loading";
import { StatusCode } from "../api/error";
import Login from "../views/login";
import { toast } from "react-toastify";

export type UserContext = {
    user: GetMeResponse;
    resetUser(): void;
};
export const USER_CONTEXT = React.createContext<UserContext>({
    user: { username: "", displayName: "", uuid: "", lastLogin: null },
    resetUser() {},
});
USER_CONTEXT.displayName = "UserContext";

type UserProviderProps = {
    children?: React.ReactNode;
};

/** Component for managing and providing the {@link UserContext} */
export function UserProvider(props: UserProviderProps) {
    type UserState = GetMeResponse | "unauthenticated" | null;
    const [user, setUser] = React.useState<UserState>(null);

    React.useEffect(() => {
        if (user == null)
            Api.user.me().then((result) =>
                result.match(
                    (user) => setUser(user),
                    (error) => {
                        switch (error.status_code) {
                            case StatusCode.Unauthenticated:
                                setUser("unauthenticated");
                                break;
                            default:
                                toast.error(error.message);
                                break;
                        }
                    }
                )
            );
    }, [user]);

    const resetUser = React.useCallback(
        function () {
            setUser(null);
        },
        [setUser]
    );

    switch (user) {
        case null:
            return <Loading />;
        case "unauthenticated":
            return <Login onLogin={resetUser} />;
        default:
            return (
                <USER_CONTEXT.Provider
                    value={{
                        user,
                        resetUser,
                    }}
                >
                    {props.children}
                </USER_CONTEXT.Provider>
            );
    }
}
