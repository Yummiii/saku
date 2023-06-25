namespace Saku.Models;

public class ChannelModel
{
    public ChannelModel(ulong discordChannelId, ChannelState state, GptModelType model)
    {
        DiscordChannelId = discordChannelId;
        State = state;
        Model = model;
    }

    public ulong DiscordChannelId { get; }
    public ChannelState State { get; }
    public string? System { get; set; }
    public GptModelType Model { get; set; }
    public int? BoundUserId { get; set; }
    public UserModel? BoundUser { get; set; }
    
    public IEnumerable<ChatContextModel>? ChatContextModels { get; set; }
}