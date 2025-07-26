"use client"

import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

interface Song {
    title: string,
}

export function MusicPlayer() {
    const [songs, setSongs] = useState<Song[]>([]);
    const [currentSong, setCurrentSong] = useState<Song | null>(null);
    const [isPlaying, setIsPlaying] = useState(false);
    const [volume, setVolume] = useState(100);

    useEffect(() => {
        async function fetchSong() {
            const songList : Song[] = await invoke("get_songs");
            setSongs(songList);
        }

        fetchSong();
    }, []);

    const playSong = async(song: Song) => {
        await invoke("play_song", { 
            title: song.title 
        });

        setCurrentSong(song);
        setIsPlaying(true);
    };

    const pauseSong = async () => {
        await invoke("pause_song");
        setIsPlaying(false);
    };

    const changeVolume = async (event: React.ChangeEvent<HTMLInputElement>) => {
        const newVolume = parseInt(event.target.value, 10);
        const normalizeVolume = newVolume / 100;
    
        setVolume(newVolume);

        await invoke("set_volume", {
            vol: normalizeVolume
        });
    };

    return (
        <div
            className="
                bg-neutral-50/30 w-100 h-1/2 flex justify-center items-center
            "
        >
            <div
                className="

                "
            >
                <h1>
                    Music Player
                </h1>
                <ul>
                    {songs.map((song) => (
                        <li
                            key={song.title}
                            onClick={() => playSong(song)}
                        >
                            <span>
                                {song.title}
                            </span>
                            {currentSong?.title === song.title && isPlaying && (
                                <span>
                                    ...
                                </span>
                            )} 
                        </li>
                    ))}
                </ul>
            {currentSong && (
                <div
                    className="

                    "
                >
                    <h2>
                        Now Playing: {currentSong.title}
                    </h2>
                    {isPlaying ? (
                        <button
                            onClick={() => pauseSong()}
                        >
                            Pause
                        </button>
                    ) : (<button
                            onClick={() => playSong(currentSong)}
                        >
                            Play
                        </button>
                    )}
                </div>
            )}
                <input 
                    type="range"
                    min={0}
                    max={100}
                    value={volume}
                    onChange={changeVolume}
                />
            </div>
        </div>
    );
};