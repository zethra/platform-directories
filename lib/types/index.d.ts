export interface BaseDir {
    homeDir: string;
    dataDir: string;
    cacheDir: string;
    configDir: string;
    dataLocalDir: string;
    executableDir?: string;
    runtimeDir?: string;
}

export declare function baseDirs(): BaseDir;