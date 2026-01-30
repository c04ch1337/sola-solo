# Sola Sensory Models

This directory contains the ONNX models required for Sola's biometric recognition capabilities.

## Required Models

### 1. YuNet Face Detector
- **File**: `face_detection_yunet_2023mar.onnx`
- **Purpose**: Detects faces in camera frames
- **Download**: https://github.com/opencv/opencv_zoo/raw/main/models/face_detection_yunet/face_detection_yunet_2023mar.onnx

### 2. SFace Face Recognizer
- **File**: `face_recognition_sface_2021dec.onnx`
- **Purpose**: Extracts 128-dimensional face embeddings
- **Download**: https://github.com/opencv/opencv_zoo/raw/main/models/face_recognition_sface/face_recognition_sface_2021dec.onnx

### 3. Speaker Encoder (ResNet-34)
- **File**: `speaker_encoder_resnet34.onnx`
- **Purpose**: Extracts speaker embeddings from Mel-spectrograms
- **Download**: https://github.com/resemble-ai/Resemblyzer/releases/download/v0.1.1/pretrained.pt (requires conversion to ONNX)
- **Alternative**: Use a pre-converted ONNX model from Hugging Face or ONNX Model Zoo

## Quick Download (PowerShell)

```powershell
cd sola-solo/backend/models

# YuNet Face Detector
Invoke-WebRequest -Uri "https://github.com/opencv/opencv_zoo/raw/main/models/face_detection_yunet/face_detection_yunet_2023mar.onnx" -OutFile "face_detection_yunet_2023mar.onnx"

# SFace Face Recognizer
Invoke-WebRequest -Uri "https://github.com/opencv/opencv_zoo/raw/main/models/face_recognition_sface/face_recognition_sface_2021dec.onnx" -OutFile "face_recognition_sface_2021dec.onnx"

# Speaker Encoder (placeholder - see note below)
# Invoke-WebRequest -Uri "URL_TO_SPEAKER_ENCODER" -OutFile "speaker_encoder_resnet34.onnx"
```

## Quick Download (curl)

```bash
cd sola-solo/backend/models

# YuNet Face Detector
curl -L -o face_detection_yunet_2023mar.onnx https://github.com/opencv/opencv_zoo/raw/main/models/face_detection_yunet/face_detection_yunet_2023mar.onnx

# SFace Face Recognizer
curl -L -o face_recognition_sface_2021dec.onnx https://github.com/opencv/opencv_zoo/raw/main/models/face_recognition_sface/face_recognition_sface_2021dec.onnx

# Speaker Encoder (placeholder - see note below)
# curl -L -o speaker_encoder_resnet34.onnx URL_TO_SPEAKER_ENCODER
```

## Model Details

### YuNet
- Input: BGR image (any size, will be resized internally)
- Output: Face bounding boxes with landmarks
- License: Apache 2.0

### SFace
- Input: Aligned face crop (112x112 BGR)
- Output: 128-dimensional embedding vector
- License: Apache 2.0

### Speaker Encoder (ResNet-34)
- Input: Mel-spectrogram (80 x 300 frames)
- Output: 256-dimensional speaker embedding (padded to 512)
- Based on: Resemblyzer / GE2E loss training
- License: MIT

## Note on Embedding Dimensions

All embeddings are padded to 512 dimensions for Qdrant compatibility:
- **SFace**: 128-dim → 512-dim (zero-padded)
- **Speaker Encoder**: 256-dim → 512-dim (zero-padded)

This allows both face and voice embeddings to coexist in the same `sola_identities` collection.

## Converting Speaker Encoder to ONNX

If you have a PyTorch speaker encoder model, convert it to ONNX:

```python
import torch
import torch.onnx

# Load your speaker encoder model
model = load_speaker_encoder()
model.eval()

# Create dummy input (batch_size=1, channels=1, n_mels=80, n_frames=300)
dummy_input = torch.randn(1, 1, 80, 300)

# Export to ONNX
torch.onnx.export(
    model,
    dummy_input,
    "speaker_encoder_resnet34.onnx",
    input_names=["mel_spectrogram"],
    output_names=["embedding"],
    dynamic_axes={"mel_spectrogram": {3: "n_frames"}},
    opset_version=11
)
```
