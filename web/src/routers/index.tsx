import { createBrowserRouter } from "react-router";

import { ErrorBoundary } from "@/components/utils/error-boundary";
import { HydrateFallback } from "@/components/utils/hydrate-fallback";

const router = createBrowserRouter([
  {
    hydrateFallbackElement: <HydrateFallback />,
    errorElement: <ErrorBoundary />,
    path: "/",
    lazy: async () => ({
      Component: (await import("@/pages/layout")).default,
    }),
    children: [
      {
        index: true,
        lazy: async () => ({
          Component: (await import("@/pages")).default,
        }),
      },
      {
        path: "users",
        children: [
          {
            path: ":user_id",
            lazy: async () => ({
              Component: (await import("@/pages/users/user_id/layout")).default,
            }),
            children: [
              {
                index: true,
                lazy: async () => ({
                  Component: (await import("@/pages/users/user_id")).default,
                }),
              },
            ],
          },
        ],
      },
      {
        path: "playground",
        children: [
          {
            index: true,
            lazy: async () => ({
              Component: (await import("@/pages/playground")).default,
            }),
          },
          {
            path: ":challenge_id",
            children: [
              {
                index: true,
                lazy: async () => ({
                  Component: (await import("@/pages/playground/challenge_id"))
                    .default,
                }),
              },
            ],
          },
        ],
      },
      {
        path: "games",
        lazy: async () => ({
          Component: (await import("@/pages/games/layout")).default,
        }),
        children: [
          {
            index: true,
            lazy: async () => ({
              Component: (await import("@/pages/games")).default,
            }),
          },
          {
            path: ":game_id",
            lazy: async () => ({
              Component: (await import("@/pages/games/game_id/layout")).default,
            }),
            children: [
              {
                index: true,
                lazy: async () => ({
                  Component: (await import("@/pages/games/game_id")).default,
                }),
              },
              {
                path: "team",
                lazy: async () => ({
                  Component: (await import("@/pages/games/game_id/team/layout"))
                    .default,
                }),
                children: [
                  {
                    index: true,
                    lazy: async () => ({
                      Component: (await import("@/pages/games/game_id/team"))
                        .default,
                    }),
                  },
                  {
                    path: "members",
                    lazy: async () => ({
                      Component: (
                        await import("@/pages/games/game_id/team/members")
                      ).default,
                    }),
                  },
                  {
                    path: "writeup",
                    lazy: async () => ({
                      Component: (
                        await import("@/pages/games/game_id/team/writeup")
                      ).default,
                    }),
                  },
                ],
              },
              {
                path: "challenges",
                lazy: async () => ({
                  Component: (await import("@/pages/games/game_id/challenges"))
                    .default,
                }),
              },
              {
                path: "scoreboard",
                lazy: async () => ({
                  Component: (await import("@/pages/games/game_id/scoreboard"))
                    .default,
                }),
              },
            ],
          },
        ],
      },
      {
        path: "account",
        children: [
          {
            path: "login",
            lazy: async () => ({
              Component: (await import("@/pages/account/login")).default,
            }),
          },
          {
            path: "register",
            lazy: async () => ({
              Component: (await import("@/pages/account/register")).default,
            }),
          },
          {
            path: "forget",
            lazy: async () => ({
              Component: (await import("@/pages/account/forget")).default,
            }),
          },
          {
            path: "idps/:idp_id",
            lazy: async () => ({
              Component: (await import("@/pages/account/idps")).default,
            }),
          },
          {
            path: "settings",
            lazy: async () => ({
              Component: (await import("@/pages/account/settings/layout"))
                .default,
            }),
            children: [
              {
                index: true,
                lazy: async () => ({
                  Component: (await import("@/pages/account/settings")).default,
                }),
              },
              {
                path: "password",
                lazy: async () => ({
                  Component: (await import("@/pages/account/settings/password"))
                    .default,
                }),
              },
              {
                path: "emails",
                lazy: async () => ({
                  Component: (await import("@/pages/account/settings/emails"))
                    .default,
                }),
              },
              {
                path: "idps",
                lazy: async () => ({
                  Component: (await import("@/pages/account/settings/idps"))
                    .default,
                }),
              },
              {
                path: "delete",
                lazy: async () => ({
                  Component: (await import("@/pages/account/settings/delete"))
                    .default,
                }),
              },
            ],
          },
        ],
      },
      {
        path: "admin",
        lazy: async () => ({
          Component: (await import("@/pages/admin/layout")).default,
        }),
        children: [
          {
            index: true,
            lazy: async () => ({
              Component: (await import("@/pages/admin")).default,
            }),
          },
          {
            path: "platform",
            lazy: async () => ({
              Component: (await import("@/pages/admin/platform")).default,
            }),
          },
          {
            path: "mailbox",
            lazy: async () => ({
              Component: (await import("@/pages/admin/mailbox")).default,
            }),
          },
          {
            path: "captcha",
            lazy: async () => ({
              Component: (await import("@/pages/admin/captcha")).default,
            }),
          },
          {
            path: "idps",
            children: [
              {
                index: true,
                lazy: async () => ({
                  Component: (await import("@/pages/admin/idps")).default,
                }),
              },
              {
                path: ":idp_id",
                lazy: async () => ({
                  Component: (await import("@/pages/admin/idps/idp_id"))
                    .default,
                }),
              },
            ],
          },
          {
            path: "challenges",
            lazy: async () => ({
              Component: (await import("@/pages/admin/challenges/layout"))
                .default,
            }),
            children: [
              {
                index: true,
                lazy: async () => ({
                  Component: (await import("@/pages/admin/challenges")).default,
                }),
              },
              {
                path: ":challenge_id",
                lazy: async () => ({
                  Component: (
                    await import("@/pages/admin/challenges/challenge_id/layout")
                  ).default,
                }),
                children: [
                  {
                    index: true,
                    lazy: async () => ({
                      Component: (
                        await import("@/pages/admin/challenges/challenge_id")
                      ).default,
                    }),
                  },
                  {
                    path: "checker",
                    lazy: async () => ({
                      Component: (
                        await import(
                          "@/pages/admin/challenges/challenge_id/checker"
                        )
                      ).default,
                    }),
                  },
                  {
                    path: "attachments",
                    lazy: async () => ({
                      Component: (
                        await import(
                          "@/pages/admin/challenges/challenge_id/attachments"
                        )
                      ).default,
                    }),
                  },
                  {
                    path: "instance",
                    lazy: async () => ({
                      Component: (
                        await import(
                          "@/pages/admin/challenges/challenge_id/instance"
                        )
                      ).default,
                    }),
                  },
                  {
                    path: "writeup",
                    lazy: async () => ({
                      Component: (
                        await import(
                          "@/pages/admin/challenges/challenge_id/writeup"
                        )
                      ).default,
                    }),
                  },
                ],
              },
            ],
          },
          {
            path: "instances",
            children: [
              {
                index: true,
                lazy: async () => ({
                  Component: (await import("@/pages/admin/instances")).default,
                }),
              },
            ],
          },
          {
            path: "games",
            lazy: async () => ({
              Component: (await import("@/pages/admin/games/layout")).default,
            }),
            children: [
              {
                index: true,
                lazy: async () => ({
                  Component: (await import("@/pages/admin/games")).default,
                }),
              },
              {
                path: ":game_id",
                lazy: async () => ({
                  Component: (
                    await import("@/pages/admin/games/game_id/layout")
                  ).default,
                }),
                children: [
                  {
                    index: true,
                    lazy: async () => ({
                      Component: (await import("@/pages/admin/games/game_id"))
                        .default,
                    }),
                  },
                  {
                    path: "challenges",
                    lazy: async () => ({
                      Component: (
                        await import("@/pages/admin/games/game_id/challenges")
                      ).default,
                    }),
                  },
                  {
                    path: "teams",
                    lazy: async () => ({
                      Component: (
                        await import("@/pages/admin/games/game_id/teams")
                      ).default,
                    }),
                  },
                  {
                    path: "notices",
                    lazy: async () => ({
                      Component: (
                        await import("@/pages/admin/games/game_id/notices")
                      ).default,
                    }),
                  },
                ],
              },
            ],
          },
          {
            path: "users",
            lazy: async () => ({
              Component: (await import("@/pages/admin/users/layout")).default,
            }),
            children: [
              {
                index: true,
                lazy: async () => ({
                  Component: (await import("@/pages/admin/users")).default,
                }),
              },
              {
                path: ":user_id",
                lazy: async () => ({
                  Component: (
                    await import("@/pages/admin/users/user_id/layout")
                  ).default,
                }),
                children: [
                  {
                    index: true,
                    lazy: async () => ({
                      Component: (await import("@/pages/admin/users/user_id"))
                        .default,
                    }),
                  },
                  {
                    path: "emails",
                    lazy: async () => ({
                      Component: (
                        await import("@/pages/admin/users/user_id/emails")
                      ).default,
                    }),
                  },
                  {
                    path: "password",
                    lazy: async () => ({
                      Component: (
                        await import("@/pages/admin/users/user_id/password")
                      ).default,
                    }),
                  },
                ],
              },
            ],
          },
        ],
      },
      {
        path: "about",
        lazy: async () => ({
          Component: (await import("@/pages/about")).default,
        }),
      },
      {
        path: "*",
        lazy: async () => ({
          Component: (await import("@/pages/sigtrap/e404")).default,
        }),
      },
    ],
  },
]);

export default router;
