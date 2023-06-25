using Discord.Interactions;

namespace Saku.ViewModels;

public enum ChannelStateInCommand
{
    [ChoiceDisplay("Desativado")]
    Disabled,
    [ChoiceDisplay("Ativado")]
    Enabled,
    [ChoiceDisplay("Sem Logs")]
    NoLogs
}