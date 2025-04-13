import wave
import numpy as np
import struct
from pathlib import Path

# Parameters for the click sound
sample_rate = 48000
duration = 0.02
frequency = 800
amplitude = 32767

# Generate the click waveform
t = np.linspace(0, duration, int(sample_rate * duration), False)
waveform = (amplitude * np.sin(2 * np.pi * frequency * t)).astype(np.int16)

# Save as a mono 16-bit WAV file
file_path = Path("assets/click.wav")
with wave.open(str(file_path), 'w') as wav_file:
    wav_file.setnchannels(1)        # mono
    wav_file.setsampwidth(2)        # 16-bit
    wav_file.setframerate(sample_rate)
    wav_file.writeframes(waveform.tobytes())

file_path.name
