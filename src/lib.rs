// Full packet ID includes 1s complement (16 bits total): 0x81_7E, 0x82_7D
// POE functions for keep alive
#[repr(u8)]
enum PacketId {
	JoinNetwork = 0x81, // S -> M
	YoureIn = 0x82, // M -> S
	HidMouseMove = 0x83,
	HidMouseButtonState = 0x84,
	HidMouseScroll = 0x85,
	HidTouch = 0x86,
	HidRelease = 0x87,
	HidKeyboardState = 0x88,
	HidControllerState = 0x89,
	SensorGyro = 0x8A,
	SensorDistance = 0x8B,
	SensorGps = 0x8C,
	SensorAccelerometer = 0x8D,
	SensorCollision = 0x8E,
	SensorOther = 0x8F,
	AudioStream = 0x90,
	VideoStream = 0x91, // Optional audio included
	DataStream = 0x92, // Generic data stream
	CompressedAudioStream = 0x93,
	CompressedVideoStream = 0x94,
	CompressedDataStream = 0x95,
	EncryptedAudioStream = 0x96, // Also compressed
	EncryptedVideoStream = 0x97, // Also compressed
	EncryptedDataStream = 0x98, // Also compressed
	AudioStreamSettingsRequest = 0x99,
	AudioStreamSettingsAccept = 0x9A,
	AudioStreamSettingsDenied = 0x9B,
	VideoStreamSettingsRequest = 0x9C,
	VideoStreamSettingsAccept = 0x9D,
	VideoStreamSettingsDenied = 0x9E,
	DataStreamSettingsRequest = 0x9F,
	DataStreamSettingsAccept = 0xA0,
	DataStreamSettingsDenied = 0xA1,
	FileDownloadRequest = 0xA2, // S -> M
	FileUploadRequest = 0xA3, // S -> M
	FileUploadAccept = 0xA4, // M -> S
	FileTransferDenied = 0xA5, // M -> S
	FileTransferData = 0xA6, // M <-> S
	DatabasePacketClient = 0xA7, // S -> M
	DatabasePacketServer = 0xA8, // M -> S
	GamePacketClient = 0xA9, // S -> M
	GamePacketServer = 0xAA, // M -> S
	SecureShellPacketClient = 0xAB, // S -> M
	SecureShellPacketServer = 0xAC, // M -> S
	ApplicationPacketClient = 0xAD, // S -> M
	ApplicationPacketServer = 0xAE, // M -> S
	WebsiteAction = 0xAF, // S -> M
	WebsiteUpdate = 0xB0, // M -> S
	FileTransferDataUnencrypted = 0xB1, // M -> S
	HidOther = 0xB2, // S -> M
}
