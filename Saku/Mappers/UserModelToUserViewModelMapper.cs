using AutoMapper;
using Lina.DynamicMapperConfiguration.Abstracts;
using Saku.Models;
using Saku.ViewModels;

namespace Saku.Mappers;

public class UserModelToUserViewModelMapper : DynamicMapperProfile<UserModel, UserViewModel>
{
    protected override void Map(IMappingExpression<UserModel, UserViewModel> mappingExpression)
    {
    }
}