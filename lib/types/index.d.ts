export interface BaseDirs {
    homeDir: string;
    dataDir: string;
    cacheDir: string;
    configDir: string;
    dataLocalDir: string;
    executableDir?: string;
    runtimeDir?: string;
}

export interface ProjectDirs {
    projectPath: string;
    dataDir: string;
    cacheDir: string;
    configDir: string;
    dataLocalDir: string;
    executableDir?: string;
    runtimeDir?: string;
}

export interface UserDirs {
    homeDir: string;
    audioDir?: string;
    desktopDir?: string;
    documentDir?: string;
    downloadDir?: string;
    fontDir?: string;
    pictureDir?: string;
    publicDir?: string;
    templateDir?: string;
    viedoDir?: string;
}

export declare function baseDirs(): BaseDirs;
export declare function projectDirs(pathOrquaifier: string,
    organization?: string, application?: string): ProjectDirs;
export declare function userDirs(): UserDirs;