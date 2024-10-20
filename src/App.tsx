import { useCallback, useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Excalidraw, MainMenu, Sidebar, WelcomeScreen, DefaultSidebar } from "@excalidraw/excalidraw";
import { AppState, ExcalidrawImperativeAPI, ExcalidrawInitialDataState } from "@excalidraw/excalidraw/types/types";
import { ExcalidrawElement } from "@excalidraw/excalidraw/types/element/types";
import { Theme } from "@tauri-apps/api/window";
import isEqual from "react-fast-compare";
import { queryOptions, useMutation, useQueryClient, useSuspenseQuery } from "@tanstack/react-query";

import "./App.css";

async function open_dialog() {
  const result = await invoke("open_dialog");
  console.log(result);
}

function importantAppState(state: AppState) {
  return {
    theme: state.theme,
    defaultSidebarDockedPreference: state.defaultSidebarDockedPreference,
    viewModeEnabled: state.viewModeEnabled,
    zenModeEnabled: state.zenModeEnabled
  }
}

export type ImportantAppState = ReturnType<typeof importantAppState>;

function appStateQuery() {
  return queryOptions({
    queryKey: ["get_app_state"],
    queryFn: async () => {
      const appState = await invoke<ImportantAppState>("get_app_state");
      //console.log('appState', appState);
      return appState;
    }
  })
}

function App() {
  const client = useQueryClient();
  const [excalidrawAPI, setExcalidrawAPI] = useState<ExcalidrawImperativeAPI | undefined>(undefined);
  let ref = useRef<HTMLElement>(null);

  const [docked, setDocked] = useState(false);

  const [name, setName] = useState<string>('');
  const { data: appState } = useSuspenseQuery(appStateQuery());
  const { mutate: setAppState } = useMutation({
    mutationKey: ["set_app_state"],
    mutationFn: async (newAppState: ImportantAppState) => {
      const result = await invoke<string>("set_app_state", { newState: newAppState });
    },
    onMutate(data) { client.setQueryData(appStateQuery().queryKey, data); }
  });

  useEffect(() => {
    console.log('name', name)
  }, [name])

  /* useEffect(() => {
    if (excalidrawAPI) {
      excalidrawAPI.getAppState().theme
    }
  }, [excalidrawAPI]) */

  const onChange = useCallback(async (elements: readonly ExcalidrawElement[], newAppState: AppState) => {
    setName(newAppState.name);
    const newState = importantAppState(newAppState);
    if (!isEqual(appState, newState)) {
      setAppState(newState);
    }
  }, [appState, setAppState]);

  return (
    <main ref={ref}>
      <Excalidraw onChange={onChange} excalidrawAPI={(api) => setExcalidrawAPI(api)} initialData={{ appState }}
        UIOptions={{ dockedSidebarBreakpoint: 1020 }}
        renderTopRightUI={() => <Sidebar.Trigger
          name="files"
          title="Files"
        >
          Files
        </Sidebar.Trigger>}>
        <WelcomeScreen />
        <MainMenu>
          <MainMenu.Item onSelect={() => open_dialog()}>Open Dialog</MainMenu.Item>
          <MainMenu.DefaultItems.LoadScene />
          <MainMenu.DefaultItems.SaveToActiveFile />
          <MainMenu.DefaultItems.SaveAsImage />
          <MainMenu.DefaultItems.Export />
          <MainMenu.Item shortcut="Shift + Alt + D" onSelect={() => open_dialog()}>Open Dialog</MainMenu.Item>
          <MainMenu.DefaultItems.Help />
          <MainMenu.DefaultItems.ClearCanvas />
          <MainMenu.Separator />
          <MainMenu.DefaultItems.Socials />
          <MainMenu.DefaultItems.ToggleTheme />
          <MainMenu.DefaultItems.ChangeCanvasBackground />
          {import.meta.env.DEV && <MainMenu.Group title="Dev Tools">
            <MainMenu.Item onSelect={() => invoke("open_devtools")}>Open Devtools</MainMenu.Item>
            <MainMenu.Item onSelect={() => invoke("reload_page")}>Reload Page</MainMenu.Item>
            <MainMenu.Item onSelect={() => invoke("restart_app")}>Restart App</MainMenu.Item>
          </MainMenu.Group>}
        </MainMenu>
        <DefaultSidebar />
        <Sidebar name="files" docked={docked} onDock={setDocked}>
          <Sidebar.Header>
            <div style={{
              color: 'var(--color-primary)',
              fontSize: '1.2em',
              fontWeight: 'bold',
              textOverflow: 'ellipsis',
              overflow: 'hidden',
              whiteSpace: 'nowrap',
              paddingRight: '1em',
            }}>Files</div>
          </Sidebar.Header>
          <p>Hello World</p>
        </Sidebar>
      </Excalidraw>
    </main >
  );
}

export default App;
